import type { Meta, StoryObj } from '@storybook/vue3';

import DropdownMenuItem from '../components/ui/dropdown-menu/DropdownMenuItem.vue';

const meta = {
  title: 'DropdownMenuItem',
  component: DropdownMenuItem,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof DropdownMenuItem>;

export default meta;
type Story = StoryObj<typeof DropdownMenuItem>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};