import type { Meta, StoryObj } from '@storybook/vue3';

import SheetContent from '../components/ui/sheet/SheetContent.vue';

const meta = {
  title: 'SheetContent',
  component: SheetContent,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof SheetContent>;

export default meta;
type Story = StoryObj<typeof SheetContent>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};