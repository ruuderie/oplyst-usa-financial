import type { Meta, StoryObj } from '@storybook/vue3';

import Sheet from '../components/ui/sheet/Sheet.vue';

const meta = {
  title: 'Sheet',
  component: Sheet,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Sheet>;

export default meta;
type Story = StoryObj<typeof Sheet>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};