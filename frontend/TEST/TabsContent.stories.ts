import type { Meta, StoryObj } from '@storybook/vue3';

import TabsContent from '../components/ui/tabs/TabsContent.vue';

const meta = {
  title: 'TabsContent',
  component: TabsContent,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof TabsContent>;

export default meta;
type Story = StoryObj<typeof TabsContent>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};